//! SPYRAL Double Helix
use super::prelude::*;

pub const ID: u32 = 1322368;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[41091257],
    listed_series: &[SET_SPYRAL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SPYRAL),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_SPYRAL as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // Change name to "SPYRAL Super Agent"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(41091257)
    effect::set_value(d, e1, 41091257);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon or add to hand 1 "SPYRAL" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsSetCard(SET_SPYRAL) and c:IsMonster() and (c:IsAbleToHand() or (zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)))
    card::is_set_card(d, c, &[SET_SPYRAL as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None)
            || (zone != 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=e:GetHandler():GetLinkedZone(tp)
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp));
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    // e:SetLabel(Duel.SelectOption(tp,70,71,72))
    let opt = duel::select_option(d, a.tp, true, &[70, 71, 72]);
    effect::set_label(d, a.e, &[opt as i64]);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)==0 then return end
    if duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(1-tp,1)
    duel::confirm_decktop(d, 1 - a.tp, 1);
    // local g=Duel.GetDecktopGroup(1-tp,1)
    let g = duel::get_decktop_group(d, 1 - a.tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local opt=e:GetLabel()
    let opt = effect::get_label(d, a.e);
    // if (opt==0 and tc:IsMonster()) or (opt==1 and tc:IsSpell()) or (opt==2 and tc:IsTrap()) then
    if (opt == 0 && card::is_monster(d, tc)) || (opt == 1 && card::is_spell(d, tc)) || (opt == 2 && card::is_trap(d, tc)) {
        // local zone=e:GetHandler():GetLinkedZone(tp)
        let h = effect::get_handler(d, a.e).nil();
        let zone = card::get_linked_zone(d, h, Some(a.tp));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,zone)
        let sg = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, zone))),
            a.tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // local sc=sg:GetFirst()
        let sc = group::get_first(d, &sg);
        // if sc then
        if let Some(sc) = sc {
            // if zone~=0 then
            if zone != 0 {
                // aux.ToHandOrElse(sc,tp,function(c)
                //     return sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end,
                // function(c)
                //     Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP,zone) end,
                // aux.Stringid(id,1))
                aux::to_hand_or_else(
                    d,
                    sc,
                    a.tp,
                    Some(&|d, _c| {
                        card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone))
                            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                    }),
                    Some(&|d, _g| {
                        duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
                        0
                    }),
                    Some(aux::stringid(ID, 1)),
                );
            } else {
                // Duel.SendtoHand(sc,nil,REASON_EFFECT)
                duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sc)
                duel::confirm_cards(d, 1 - a.tp, sc);
            }
        }
    }
}
