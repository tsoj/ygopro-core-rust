//! Gem-Knight Lady Lapis Lazuli
use super::prelude::*;

pub const ID: u32 = 47611119;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GEM_KNIGHT],
    material_setcode: &[SET_GEM, SET_GEM_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,false,false,99645428,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GEM_KNIGHT))
    fusion::add_proc_mix(
        d,
        c,
        false,
        false,
        vec![
            fusion::Mat::Code(99645428),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GEM_KNIGHT as u16], Some(fc), st, Some(tp)))),
        ],
    );
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DAMAGE+CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DAMAGE + CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA) or (st&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
    let h = effect::get_handler(d, a.e(0)).nil();
    let st = a.int(3) as u32;
    !card::is_location(d, h, LOCATION_EXTRA) || (st & SUMMON_TYPE_FUSION as u32) == SUMMON_TYPE_FUSION as u32
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GEM_KNIGHT) and c:IsMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned()
    card::is_special_summoned(d, c)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.ctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&ctfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(s.ctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&ctfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (ct * 500) as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
        let og = duel::get_operated_group(d);
        let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
        // if ct>0 then
        if ct > 0 {
            // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
            let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
            let p = info[0].int() as u8;
            // local ct=Duel.GetMatchingGroupCount(s.ctfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
            let ct = duel::get_matching_group_count(d, Some(&ctfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
            // Duel.Damage(p,ct*500,REASON_EFFECT)
            duel::damage(d, p, (ct * 500) as i64, REASON_EFFECT, false, None);
        }
    }
}
