//! Ultra Polymerization
use super::prelude::*;

pub const ID: u32 = 76647978;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion.CreateSummonEff(c,nil,Card.IsOnField,nil,nil,nil,s.stage2,2)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_on_field(d, c))),
            stage2: Some(std::sync::Arc::new(stage2)),
            exactcount: Some(2),
            ..Default::default()
        },
    );
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    let tg = effect::get_target(d, e1).nil();
    effect::set_target(
        d,
        e1,
        Some(LuaFn::new(move |d, a| {
            // function(e,tp,eg,ep,ev,re,r,rp,chk)
            let chk = a.get(8).map_or(0, |v| v.int());
            if chk == 0 {
                return tg.call(d, a);
            }
            tg.call(d, a);
            let e = a.first().and_then(|v| v.as_effect()).nil();
            if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
                duel::set_chain_limit(d, aux_false());
            }
            Value::Nil
        })),
    );
    card::register_effect(d, c, e1, false);
    // Special Summon from the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.stage2(e,tc,tp,sg,chk)`
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _sg: Option<&CardSet>, chk: i64) {
    if chk == 1 {
        let tc = tc.nil();
        let h = effect::get_handler(d, e).nil();
        let cid = card::get_card_id(d, h);
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, cid as i64, 0);
    }
}
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fusc: CardId, mg: &CardSet) -> bool {
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & (REASON_FUSION | REASON_MATERIAL)) == (REASON_FUSION | REASON_MATERIAL)
        && card::get_reason_card(d, c) == Some(fusc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::check_fusion_material(d, fusc, Some(mg), Some(&CardSet::from(c)), Some((PLAYER_NONE | FUSPROC_NOTFUSION) as u64))
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if card::is_faceup(d, c) && card::get_flag_effect(d, c, ID) != 0 && {
        let h = effect::get_handler(d, e).nil();
        let cid = card::get_card_id(d, h);
        card::get_flag_effect_label(d, c, ID).first().copied() == Some(cid as i64)
    } {
        let mg = card::get_material(d, c);
        let ct = mg.len() as i32;
        ct > 0
            && ct <= duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
            && group::filter_count(d, &mg, &|d, mc| mgfilter(d, mc, e, tp, c, &mg), NO_EX) == ct as usize
            && (duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none() || ct <= 1)
    } else {
        false
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let _ = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    let c = effect::get_handler(d, e).nil();
    let Some(tc) = duel::first_target(d) else { return };
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    let mg = card::get_material(d, tc);
    let ct = mg.len() as i32;
    if ct > 0
        && ct <= duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
        && group::filter_count(d, &mg, &|d, mc| mgfilter(d, mc, e, tp, tc, &mg), NO_EX) == ct as usize
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
    {
        // local sc=mg:GetFirst()
        let _ = group::get_first(d, &mg);
        for sc in group::iter(d, &mg) {
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_DISABLE);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, sc, e1, true);

                let e2 = effect::create_effect(d, c);
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, sc, e2, true);

                let e3 = effect::create_effect(d, c);
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
                effect::set_value(d, e3, 0);
                effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, sc, e3, true);

                let e4 = effect::clone(d, e3);
                effect::set_code(d, e4, EFFECT_SET_DEFENSE_FINAL);
                card::register_effect(d, sc, e4, true);
            }
        }
        duel::special_summon_complete(d);
    }
}
