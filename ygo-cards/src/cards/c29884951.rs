//! Swordsoul Auspice Chunjun
use super::prelude::*;

pub const ID: u32 = 29884951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy itself and an opponent's monster that battles your Wyrm monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Special Summon itself from hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(is_main_phase)));
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Banish 1 card from the field or GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_REMOVE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(rmcon)));
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    effect::set_operation(d, e3, Some(operation(rmop)));
    card::register_effect(d, c, e3, false);
}
/// `s.destg(e,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local a,b=Duel.GetAttacker(),Duel.GetAttackTarget()
    let mut at = duel::get_attacker(d);
    let mut bt = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then a,b=b,a end
    if card::is_controler(d, at.nil(), 1 - a.tp) {
        std::mem::swap(&mut at, &mut bt);
    }
    // if chk==0 then return a and a:IsRace(RACE_WYRM) and b and b:IsControler(1-tp) and b:IsSummonLocation(LOCATION_EXTRA) end
    if a.chk == Some(0) {
        return match (at, bt) {
            (Some(x), Some(y)) => {
                card::is_race(d, x, RACE_WYRM) && card::is_controler(d, y, 1 - a.tp) && card::is_summon_location(d, y, LOCATION_EXTRA)
            }
            _ => false,
        };
    }
    // local g=Group.FromCards(e:GetHandler(),b)
    let h = effect::get_handler(d, a.e).nil();
    let mut g = CardSet::from_card(h);
    if let Some(bb) = bt {
        group::add_card(&mut g, &CardSet::from_card(bb));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local a,b=Duel.GetAttacker(),Duel.GetAttackTarget()
    let mut at = duel::get_attacker(d);
    let mut bt = duel::get_attack_target(d);
    // if a:IsStatus(STATUS_ATTACK_CANCELED) then return end
    if card::is_status(d, at.nil(), STATUS_ATTACK_CANCELED) {
        return;
    }
    // if a:IsControler(1-tp) then a,b=b,a end
    if card::is_controler(d, at.nil(), 1 - a.tp) {
        std::mem::swap(&mut at, &mut bt);
    }
    // if a and a:IsRelateToBattle() and b and b:IsRelateToBattle() and b:IsControler(1-tp) then
    if let Some(x) = at {
        if card::is_relate_to_battle(d, x) {
            if let Some(y) = bt {
                if card::is_relate_to_battle(d, y) && card::is_controler(d, y, 1 - a.tp) {
                    // local g=Group.FromCards(c,b)
                    let mut g = CardSet::from_card(c);
                    group::add_card(&mut g, &CardSet::from_card(y));
                    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
                    if !g.is_empty() {
                        duel::destroy(d, g, REASON_EFFECT, None, None);
                    }
                }
            }
        }
    }
}
/// `s.spcost(e,tp,...,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.CheckReleaseGroupCost(tp,nil,1,false,aux.ReleaseCheckMMZ,nil)
        return duel::check_release_group_cost(
            d,
            a.tp,
            None,
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)),
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,aux.ReleaseCheckMMZ,nil)
    let g = duel::select_release_group_cost(
        d,
        a.tp,
        None,
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_mmz(d, sg, tp, exg)),
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,...,chk)` (Chunjun's own)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function() return Duel.IsMainPhase() end`
fn is_main_phase(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && a.r == REASON_SYNCHRO
}
/// `s.rmtg(e,...,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD|LOCATION_GRAVE) and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
            a.tp,
            LOCATION_ONFIELD | LOCATION_GRAVE,
            LOCATION_ONFIELD | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
/// `s.rmop(e,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Remove(tc,POS_FACEUP,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
