//! Eater of Millions
use super::prelude::*;

pub const ID: u32 = 63845230;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Increase ATK/DEF
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.val)
    effect::set_value_fn(d, e3, effect_card_value(val));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Cannot be Tributed
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e5, EFFECT_UNRELEASABLE_SUM);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetCode(EFFECT_UNRELEASABLE_NONSUM)
    effect::set_code(d, e6, EFFECT_UNRELEASABLE_NONSUM);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Cannot be used as material for a Fusion/Synchro/Xyz Summon
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e7, EFFECT_CANNOT_BE_MATERIAL);
    // e7:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ))
    effect::set_value_fn(d, e7, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ]));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // Banish battling monster
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e8, aux::stringid(ID, 0));
    // e8:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e8, CATEGORY_REMOVE);
    // e8:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e8:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e8, EVENT_BATTLE_START);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetTarget(s.rmtg)
    effect::set_target(d, e8, Some(cost_target(rmtg)));
    // e8:SetOperation(s.rmop)
    effect::set_operation(d, e8, Some(operation(rmop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_EXTRA,0,c,POS_FACEDOWN)
    let rg = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_able_to_remove_as_cost(d, x, Some(POS_FACEDOWN as u8))),
        tp,
        LOCATION_HAND | LOCATION_ONFIELD | LOCATION_EXTRA,
        0,
        c.into(),
    );
    // return Duel.GetLocationCount... (Duel.GetMZoneCount(tp,rg)>0 and #rg>4 and aux.SelectUnselectGroup(rg,e,tp,5,#rg,aux.ChkfMMZ(1),0))
    if !(duel::get_m_zone_count(d, tp, (&rg).into(), None, None, None).0 > 0) {
        return false;
    }
    if !(rg.len() > 4) {
        return false;
    }
    let n = rg.len();
    matches!(
        aux::select_unselect_group(d, &rg, Some(e), tp, Some(5), Some(n), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_EXTRA,0,c,POS_FACEDOWN)
    let rg = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_able_to_remove_as_cost(d, x, Some(POS_FACEDOWN as u8))),
        tp,
        LOCATION_HAND | LOCATION_ONFIELD | LOCATION_EXTRA,
        0,
        c.into(),
    );
    // local g1=Group.CreateGroup()
    // local g2=Group.CreateGroup()
    // (new groups: assigned below)
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    if ft <= 0 {
        // g1=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_MMZONE,0,1,1,true,c,POS_FACEDOWN)
        let g1 = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| card::is_able_to_remove_as_cost(d, x, Some(POS_FACEDOWN as u8))),
            tp,
            LOCATION_MMZONE,
            0,
            1,
            1,
            true,
            c.into(),
        );
        // if g1 and #g1>0 then
        if let Some(mut g1) = g1 {
            if !g1.is_empty() {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
                // g2=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_EXTRA,0,4,#rg,true,Group.FromCards(c,g1:GetFirst()),POS_FACEDOWN)
                let first = group::get_first(d, &g1).nil();
                let mut ex = CardSet::from_card(c);
                group::add_card(&mut ex, &CardSet::from_card(first));
                let max = rg.len() as u16;
                let g2 = duel::select_matching_card(
                    d,
                    tp,
                    Some(&|d, x| card::is_able_to_remove_as_cost(d, x, Some(POS_FACEDOWN as u8))),
                    tp,
                    LOCATION_HAND | LOCATION_ONFIELD | LOCATION_EXTRA,
                    0,
                    4,
                    max,
                    true,
                    (&ex).into(),
                );
                // if g2 and #g2>0 then
                if let Some(g2) = g2 {
                    if !g2.is_empty() {
                        // g1:Merge(g2)
                        group::add_card(&mut g1, &g2);
                        // g1:KeepAlive() (groups are values in Rust)
                        // e:SetLabelObject(g1)
                        effect::set_label_object(d, e, Value::Group(g1));
                        // return true
                        return true;
                    }
                }
            }
        }
    } else {
        // g1=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_EXTRA,0,5,#rg,true,c,POS_FACEDOWN)
        let max = rg.len() as u16;
        let g1 = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| card::is_able_to_remove_as_cost(d, x, Some(POS_FACEDOWN as u8))),
            tp,
            LOCATION_HAND | LOCATION_ONFIELD | LOCATION_EXTRA,
            0,
            5,
            max,
            true,
            c.into(),
        );
        // if g1 and #g1>0 then
        if let Some(g1) = g1 {
            if !g1.is_empty() {
                // g1:KeepAlive() (groups are values in Rust)
                // e:SetLabelObject(g1)
                effect::set_label_object(d, e, Value::Group(g1));
                // return true
                return true;
            }
        }
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEDOWN,REASON_COST)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None);
    // g:DeleteGroup() (nothing in Rust)
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsFacedown,0,LOCATION_REMOVED,LOCATION_REMOVED,nil)*100
    duel::get_matching_group_count(d, Some(&|d, c| card::is_facedown(d, c)), 0, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX) as i64 * 100
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // if chk==0 then return tc and tc:IsControler(1-tp) and tc:IsAbleToRemove(tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return tc.is_some_and(|tc| {
            card::is_controler(d, tc, 1 - a.tp) && card::is_able_to_remove(d, tc, Some(a.tp), Some(POS_FACEDOWN as u8), None)
        });
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // if tc and tc:IsRelateToBattle() then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) {
            // Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        }
    }
}
