//! Gunkan Suship Ikura
use super::prelude::*;

pub const ID: u32 = 61027400;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SUSHIP_SHARI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure (from hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Excavate and either Special Summon or add to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.exctg)
    effect::set_target(d, e2, Some(cost_target(exctg)));
    // e2:SetOperation(s.excop)
    effect::set_operation(d, e2, Some(operation(excop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SUSHIP_SHARI),c:GetControler(),LOCATION_ONFIELD,0,1,nil)
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_SUSHIP_SHARI])), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.exctg(e,tp,...,chk)`
fn exctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>2 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 2;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.excfilter(c,e,tp)`
fn excfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_SUSHIP_SHARI) and (c:IsAbleToHand() or c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    card::is_code(d, c, &[CARD_SUSHIP_SHARI]) && (card::is_able_to_hand(d, c, None) || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.excop(e,tp,...)`
fn excop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<3 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 3 {
        return;
    }
    // Duel.ConfirmDecktop(tp,3)
    duel::confirm_decktop(d, a.tp, 3);
    // local g=Duel.GetDecktopGroup(tp,3):Filter(s.excfilter,nil,e,tp)
    let decktop = duel::get_decktop_group(d, a.tp, 3);
    let g = group::filter(d, &decktop, &|d, c| excfilter(d, c, a.e, a.tp), NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // local tc=sg:GetFirst()
        let tc = group::get_first(d, &sg).nil();
        // aux.ToHandOrElse(tc,tp,function(c)
        //     return tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end,
        //     function(c) Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,3))
        let (e, tp) = (a.e, a.tp);
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&move |d: &mut Duel, _c: CardId| {
                card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            }),
            Some(&move |d: &mut Duel, _c: &CardSet| duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
            Some(aux::stringid(ID, 3)),
        );
    } else {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
    }
}
