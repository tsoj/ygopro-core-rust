//! Trishula, the Dragon of Icy Imprisonment
use super::prelude::*;

pub const ID: u32 = 15661378;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon/Contact Fusion
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local eff=Fusion.AddProcMixN(c,true,true,s.ffilter,3)
    let eff = fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, x, a| ffilter(d, x, a))), 3)]);
    // if not c:IsStatus(STATUS_COPYING_EFFECT) then
    if !card::is_status(d, c, STATUS_COPYING_EFFECT) {
        // eff[1]:SetValue(s.matfilter)
        effect::set_value_fn(d, eff.nil(), lua_fn(matfilter));
    }
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit,nil,nil,nil,false)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            cannot_be_lizard: Some(false),
            ..Default::default()
        },
    );
    // Banish 1 card each from your Deck, the top of the opponent's deck, and the opponent's Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.remcon)
    effect::set_condition(d, e1, Some(condition(remcon)));
    // e1:SetTarget(s.remtg)
    effect::set_target(d, e1, Some(cost_target(remtg)));
    // e1:SetOperation(s.remop)
    effect::set_operation(d, e1, Some(operation(remop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ffilter(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,sumtype,tp),fc,tp))
    match a.sg {
        None => true,
        Some(sg) => {
            // (c:GetCode(fc,sumtype,tp) is not the last argument: only its first code is passed)
            let code = card::get_code_ctx(d, c, Some(a.fc), a.sumtype, Some(a.tp)).first().copied().unwrap_or(0);
            !group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, a.fc, a.tp), 1, c.into())
        }
    }
}
/// `s.fusfilter(c,code,fc,tp)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, tp: u8) -> bool {
    // return c:IsSummonCode(fc,SUMMON_TYPE_FUSION,tp,code) and not c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), SUMMON_TYPE_FUSION as u64, tp, &[code]) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `s.matfilter(c,fc,sub,sub2,mg,sg,tp,contact,sumtype)`
fn matfilter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    let fc = a.c(1);
    let tp = a.p(6);
    let contact = a.v(7).truthy();
    let sumtype = a.int(8) as u64;
    // if sumtype&SUMMON_TYPE_FUSION~=0 and fc:IsLocation(LOCATION_EXTRA) and not contact then
    if sumtype & SUMMON_TYPE_FUSION as u64 != 0 && card::is_location(d, fc, LOCATION_EXTRA) && !contact {
        // return c:IsLocation(LOCATION_ONFIELD|LOCATION_HAND) and c:IsControler(tp)
        return card::is_location(d, c, LOCATION_ONFIELD | LOCATION_HAND) && card::is_controler(d, c, tp);
    }
    // return true
    true
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(s.filteraux,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&filteraux), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.filteraux(c)`
fn filteraux(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemoveAsCost() and c:IsMonster()
    card::is_able_to_remove_as_cost(d, c, None) && card::is_monster(d, c)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetLocation()~=LOCATION_EXTRA or (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
    let h = effect::get_handler(d, a.e(0)).nil();
    let st = a.int(3) as u64;
    card::get_location(d, h) != LOCATION_EXTRA || (st & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetOriginalRace()~=RACE_DRAGON
    card::get_original_race(d, c) != RACE_DRAGON
}
/// `s.remcon(e,tp,eg,ep,ev,re,r,rp)`
fn remcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local mg=e:GetHandler():GetMaterial()
    // return mg and e:GetHandler():IsSummonLocation(LOCATION_EXTRA) and not mg:IsExists(s.mfilter,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    let mg = card::get_material(d, h);
    card::is_summon_location(d, h, LOCATION_EXTRA) && !group::is_exists(d, &mg, &|d, x| mfilter(d, x), 1, NO_EX)
}
/// `Card.IsAbleToRemove` as a filter (no extra arguments)
fn able_to_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `s.remtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn remtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_DECK,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, 0, LOCATION_DECK, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,0,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.remop(e,tp,eg,ep,ev,re,r,rp)`
fn remop(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_DECK,0,nil)
    let g1 = duel::get_matching_group(d, Some(&able_to_remove), a.tp, LOCATION_DECK, 0, NO_EX);
    // local g2=Duel.GetDecktopGroup(1-tp,1)
    let g2 = duel::get_decktop_group(d, 1 - a.tp, 1);
    // local g3=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,nil)
    let g3 = duel::get_matching_group(d, Some(&able_to_remove), a.tp, 0, LOCATION_EXTRA, NO_EX);
    // if #g1>0 and #g2>0 and #g3>0 then
    if !g1.is_empty() && !g2.is_empty() && !g3.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg1=g1:Select(tp,1,1,nil)
        let mut sg1 = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmDecktop(1-tp,1)
        duel::confirm_decktop(d, 1 - a.tp, 1);
        // Duel.ConfirmCards(tp,g3)
        duel::confirm_cards(d, a.tp, g3.clone());
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg3=g3:Select(tp,1,1,nil)
        let sg3 = group::select(d, &g3, a.tp, 1, 1, false, NO_EX).nil();
        // sg1:Merge(g2)
        group::add_card(&mut sg1, &g2);
        // sg1:Merge(sg3)
        group::add_card(&mut sg1, &sg3);
        // Duel.Remove(sg1,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg1, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // Duel.ShuffleExtra(1-tp)
        duel::shuffle_extra(d, 1 - a.tp);
    }
}
