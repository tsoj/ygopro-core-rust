//! Utopic Onomatopoeia
use super::prelude::*;

pub const ID: u32 = 8512558;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={id}
    listed_names: &[ID],
    // s.listed_series={SET_ZUBABA,SET_GAGAGA,SET_GOGOGO,SET_DODODO}
    listed_series: &[SET_ZUBABA as u32, SET_GAGAGA as u32, SET_GOGOGO as u32, SET_DODODO as u32],
    ..ScriptInfo::DEFAULT
};

// {SET_ZUBABA,SET_GAGAGA,SET_GOGOGO,SET_DODODO}
const SETS: [u16; 4] = [SET_ZUBABA as u16, SET_GAGAGA as u16, SET_GOGOGO as u16, SET_DODODO as u16];
// aux.PropertyTableFilter(Card.GetSetCard, SET_ZUBABA, SET_GAGAGA, SET_GOGOGO, SET_DODODO)
const ALLOWED: &[i64] = &[SET_ZUBABA as i64, SET_GAGAGA as i64, SET_GOGOGO as i64, SET_DODODO as i64];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card is always treated as a "Zubaba" card (required due to current cdb limitations)
    // c:AddSetcodesRule(id,true,SET_ZUBABA)
    card::add_setcodes_rule(d, c, ID, true, &[SET_ZUBABA as u16]);
    // Special Summon up to 1 each "Zubaba", "Gagaga", "Gogogo", and/or "Dododo" monster(s) from your hand in Defense Position, except "Utopic Onomatopoeia"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_ZUBABA,SET_GAGAGA,SET_GOGOGO,SET_DODODO}) and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &SETS)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// Card.GetSetCard as a list of values (for aux.PropertyTableFilter)
fn setcodes(d: &mut Duel, c: CardId) -> Vec<i64> {
    card::get_set_card(d, c).into_iter().map(|x| x as i64).collect()
}
// function(sg,e,tp,mg) return true, not aux.ChkfMMZ(#sg)(sg,e,tp,mg) or not sg:CheckDifferentProperty(checkfunc) end
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    let mmz = aux::chkf_mmz(sg.len() as i32)(d, sg, e, tp, mg, c).0;
    let checkfunc = aux::property_table_filter(setcodes, ALLOWED);
    (true, !mmz || !group::check_different_property(d, sg, &checkfunc))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>0 and aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon(...),0) end
    if a.chk == Some(0) {
        return ft > 0
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),4)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0.min(4);
    // if #g>0 and ft>0 then
    if !g.is_empty() && ft > 0 {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon(checkfunc),1,tp,HINTMSG_SPSUMMON,s.rescon(checkfunc))
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(a.e),
            a.tp,
            Some(1),
            Some(ft as usize),
            Some(&rescon),
            1,
            a.tp,
            Some(HINTMSG_SPSUMMON as u64),
            Some(&rescon),
            None,
            false,
        )
        .unwrap_or_default();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_XYZ) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_XYZ))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // "Clock Lizard" check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalType(TYPE_XYZ) end)
    aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_XYZ))), None, None, None, None);
}
