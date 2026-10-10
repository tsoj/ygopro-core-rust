//! Dark Magician the Knight of Dragon Magic
use super::prelude::*;

pub const ID: u32 = 5829717;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_GAIA_CHAMPION], ..ScriptInfo::DEFAULT };

/// `LOCATION_HAND_DECK_GRAVE_EXTRA` (a local of the Lua script)
const LOCATION_HAND_DECK_GRAVE_EXTRA: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_EXTRA;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Material
    // Fusion.AddProcMix(c,true,true,CARD_DARK_MAGICIAN,s.ffilter)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(CARD_DARK_MAGICIAN), fusion::Mat::F(fusion::fus_mat(ffilter))]);
    // Your monsters deal piercing damage
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_PIERCE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    card::register_effect(d, c, e1, false);
    // Inflict damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Dark Magician" and 1 "Gaia the Dragon Champion"
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, st: u64, tp: u8) -> bool {
    // return c:IsLevelAbove(7) and c:IsRace(RACE_DRAGON|RACE_WARRIOR,fc,sumtype,tp)
    card::is_level_above(d, c, 7) && card::is_race_ctx(d, c, RACE_DRAGON | RACE_WARRIOR, Some(fc), st, Some(tp))
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=eg:Filter(Card.IsPreviousControler,nil,1-tp)
    let eg = a.eg.clone().nil();
    let dg = group::filter(d, &eg, &|d, c| card::is_previous_controler(d, c, 1 - a.tp), NO_EX);
    // if #dg==0 or #dg>1 then return false end
    if dg.len() == 0 || dg.len() > 1 {
        return false;
    }
    // local rc=dg:GetFirst():GetReasonCard()
    let first = group::get_first(d, &dg).nil();
    let rc = card::get_reason_card(d, first).nil();
    // if rc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, rc) {
        // return rc:IsControler(tp)
        card::is_controler(d, rc, a.tp)
    } else {
        // return rc:IsPreviousControler(tp)
        card::is_previous_controler(d, rc, a.tp)
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local dc=eg:Filter(Card.IsPreviousControler,nil,1-tp):GetFirst()
    let eg = a.eg.clone().nil();
    let filtered = group::filter(d, &eg, &|d, c| card::is_previous_controler(d, c, 1 - a.tp), NO_EX);
    let dc = group::get_first(d, &filtered);
    // if chk==0 then return dc and dc:GetBaseAttack()>0 end
    if a.chk == Some(0) {
        return match dc {
            Some(dc) => card::get_base_attack(d, dc) > 0,
            None => false,
        };
    }
    // local dam=dc:GetBaseAttack()
    let dam = card::get_base_attack(d, dc.nil());
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_DARK_MAGICIAN,CARD_GAIA_CHAMPION) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_GAIA_CHAMPION]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsCode,nil,CARD_DARK_MAGICIAN)==1
    (group::filter_count(d, sg, &|d, c| card::is_code(d, c, &[CARD_DARK_MAGICIAN]), NO_EX) == 1, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND_DECK_GRAVE_EXTRA,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND_DECK_GRAVE_EXTRA, 0, NO_EX);
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND_DECK_GRAVE_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_HAND_DECK_GRAVE_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND_DECK_GRAVE_EXTRA,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_HAND_DECK_GRAVE_EXTRA, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
