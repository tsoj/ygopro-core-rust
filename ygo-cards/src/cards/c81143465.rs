//! あないみじや玉の緒ふたつ
//! Tragic Twin Twined Jewels
use super::prelude::*;

pub const ID: u32 = 81143465;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip only to a monster you control that was Special Summoned from the Extra Deck
    // aux.AddEquipProcedure(c,0,aux.FilterBoolFunction(Card.IsSummonLocation,LOCATION_EXTRA))
    aux::add_equip_procedure(d, c, Some(0), Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_summon_location(d, c, LOCATION_EXTRA))), None, None, None, None, None, None);

    // Destroy it, also the equipped monster, and if you do, take damage equal to the total original ATK of the destroyed monsters, then inflict damage to your opponent equal to the damage you took
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
fn desconfilter(d: &mut Duel, c: CardId, tp: u8, atk: i32) -> bool {
    // c:IsSummonPlayer(1-tp) and c:IsSummonLocation(LOCATION_EXTRA) and c:GetAttack()>atk
    card::is_summon_player(d, c, 1 - tp) && card::is_summon_location(d, c, LOCATION_EXTRA) && card::get_attack(d, c) > atk
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // return ec and eg:IsExists(s.desconfilter,1,nil,tp,ec:GetAttack())
    match ec {
        None => false,
        Some(ec) => {
            let eg = a.eg();
            let atk = card::get_attack(d, ec);
            group::is_exists(d, &eg, &|d, c| desconfilter(d, c, a.tp, atk), 1, NO_EX)
        }
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // local g=eg:Filter(s.desconfilter,nil,tp,ec:GetAttack()):Match(Card.IsCanBeEffectTarget,nil,e)
    let eg = a.eg();
    let atk = card::get_attack(d, ec);
    let mut g = group::filter(d, &eg, &|d, c| desconfilter(d, c, a.tp, atk), NO_EX);
    group::match_(d, &mut g, &|d, c| card::is_can_be_effect_target(d, c, Some(a.e)), NO_EX);
    // if chkc then return g:IsContains(chkc) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tg=nil
    // if #g==1 then
    let tg = if g.len() == 1 {
        // tg=g
        g.clone()
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // tg=g:Select(tp,1,1,nil)
        group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil()
    };
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg+ec,2,tp,0)
    let mut dg = tg.clone();
    group::add_card(&mut dg, &CardSet::from_card(ec));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 2, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,tg:GetSum(Card.GetBaseAttack))
    let sum = group::get_sum(d, &tg, &|d, c| card::get_base_attack(d, c) as i64);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, sum as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local g=Group.FromCards(e:GetHandler():GetEquipTarget())
    let h = effect::get_handler(d, a.e).nil();
    let mut g = match card::get_equip_target(d, h) {
        Some(ec) => CardSet::from_card(ec),
        None => CardSet::new(),
    };
    // if tc:IsRelateToEffect(e) then
    let tc = tc.nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // g:AddCard(tc)
        group::add_card(&mut g, &CardSet::from_card(tc));
    }
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local dam=Duel.GetOperatedGroup():GetSum(Card.GetBaseAttack)
        let op = duel::get_operated_group(d);
        let dam = group::get_sum(d, &op, &|d, c| card::get_base_attack(d, c) as i64);
        // if dam==0 then return end
        if dam == 0 {
            return;
        }
        // local val=Duel.Damage(tp,dam,REASON_EFFECT)
        let val = duel::damage(d, a.tp, dam, REASON_EFFECT, false, None);
        // if val>0 then
        if val > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,val,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, val as i64, REASON_EFFECT, false, None);
        }
    }
}
