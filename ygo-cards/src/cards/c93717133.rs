//! Galaxy-Eyes Photon Dragon
use super::prelude::*;

pub const ID: u32 = 93717133;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetHintTiming(TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e2, TIMING_BATTLE_PHASE, None);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(c) return c:IsAttackAbove(2000) end` (the filter of `Duel.CheckReleaseGroup` and `SelectReleaseGroup`)
fn atk_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_above(d, c, 2000)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.CheckReleaseGroup(c:GetControler(),Card.IsAttackAbove,2,false,2,true,c,c:GetControler(),nil,false,nil,2000)
    let tp = card::get_controler(d, c);
    duel::check_release_group(
        d,
        tp,
        Some(&atk_filter),
        2,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(2), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // the 9th argument of an EFFECT_SPSUMMON_PROC target is chk (1), not the card: `c` is a number in the Lua,
    // so `to_check` of SelectReleaseGroup is nil there
    // local g=Duel.SelectReleaseGroup(tp,Card.IsAttackAbove,2,2,false,true,true,c,nil,nil,false,nil,2000)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&atk_filter),
        2,
        2,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: None, ..Default::default() },
        NO_EX,
        false,
    );
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
        // g:DeleteGroup()
    }
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and not e:GetHandler():IsStatus(STATUS_CHAINING)
    let h = effect::get_handler(d, a.e).nil();
    duel::is_battle_phase(d, None) && !card::is_status(d, h, STATUS_CHAINING)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if chk==0 then return bc and bc:IsOnField() and bc:IsCanBeEffectTarget(e) and c:IsAbleToRemove() and bc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return match bc {
            None => false,
            Some(bc) => {
                card::is_on_field(d, bc)
                    && card::is_can_be_effect_target(d, bc, Some(e))
                    && card::is_able_to_remove(d, c, None, None, None)
                    && card::is_able_to_remove(d, bc, None, None, None)
            }
        };
    }
    // Duel.SetTargetCard(bc)
    if let Some(bc) = bc {
        duel::set_target_card(d, bc);
    }
    // local g=Group.FromCards(c,bc)
    let mut cards = vec![c];
    if let Some(bc) = bc {
        cards.push(bc);
    }
    let g = group::create_group(&cards);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 2, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) || !card::is_relate_to_effect(d, tc.nil(), e) {
        return;
    }
    let tc = tc.nil();
    // local g=Group.FromCards(c,tc)
    let g = group::create_group(&[c, tc]);
    // local mcount=0
    let mut mcount: i64 = 0;
    // if tc:IsFaceup() then mcount=tc:GetOverlayCount() end
    if card::is_faceup(d, tc) {
        mcount = card::get_overlay_count(d, tc) as i64;
    }
    // if Duel.Remove(g,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if duel::remove(d, g, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if not og:IsContains(tc) then mcount=0 end
        if !group::is_contains(&og, tc) {
            mcount = 0;
        }
        // local oc=og:GetFirst()
        let _ = group::get_first(d, &og);
        // for oc in aux.Next(og) do
        for oc in group::iter(d, &og) {
            // oc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            card::register_flag_effect(d, oc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
        // og:KeepAlive()
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_BATTLE)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE, 1);
        // e1:SetLabel(mcount)
        effect::set_label(d, e1, &[mcount]);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabelObject(og)
        effect::set_label_object(d, e1, Value::Group(og));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.retfilter(c)`
fn retfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = match effect::get_label_object(d, e) {
        Value::Group(g) => g,
        _ => crate::lua_error("attempt to index a nil value"),
    };
    // local sg=g:Filter(s.retfilter,nil)
    let sg = group::filter(d, &g, &retfilter, NO_EX);
    // g:DeleteGroup()
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // if Duel.ReturnToField(tc) and tc==e:GetOwner() and tc:IsFaceup() and e:GetLabel()~=0 then
        if duel::return_to_field(d, tc, None, None)
            && effect::get_owner(d, e) == Some(tc)
            && card::is_faceup(d, tc)
            && effect::get_label(d, e) != 0
        {
            // local e1=Effect.CreateEffect(e:GetOwner())
            let owner = effect::get_owner(d, e).nil();
            let e1 = effect::create_effect(d, owner);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // e1:SetValue(e:GetLabel()*500)
            effect::set_value(d, e1, effect::get_label(d, e) * 500);
            // e:GetOwner():RegisterEffect(e1)
            card::register_effect(d, owner, e1, false);
        }
    }
}
